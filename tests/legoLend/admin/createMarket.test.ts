import * as anchor from "@coral-xyz/anchor";
import { AnchorError } from "@coral-xyz/anchor";
import { assert } from "chai";

import { bumpRangeInclusive, decimals, errors } from "../../utils/constants";
import { createSplTokenMint, pda, programMethods, transfer } from "../../utils/utils";
import { setup } from "../../utils/setup";
import { getAccount } from "@solana/spl-token";

describe("LegoLend", () => {
    const { provider, owner, feeRecipient, irmBase, legoLend, oracleBase } = setup();
    const newOwner = anchor.web3.Keypair.generate();
    let usdcMint: anchor.web3.PublicKey, wbtcMint: anchor.web3.PublicKey;

    before(async () => {
        usdcMint = await createSplTokenMint(provider.connection, owner, decimals);
        wbtcMint = await createSplTokenMint(provider.connection, owner, decimals);

        await programMethods.legoLend.initialize(owner, feeRecipient.publicKey, legoLend);
    });

    it("Market creation fails if caller is not owner", async () => {
        const lltv = new anchor.BN(9e8);
        const fee = new anchor.BN(1e8);
        const market = pda.legoLend.getMarket(usdcMint, wbtcMint, legoLend);
        const loanTokenAccount = pda.legoLend.getVault(market, usdcMint, legoLend);
        const collateralTokenAccount = pda.legoLend.getVault(market, wbtcMint, legoLend);

        const airdropAmount = 3e9;
        await transfer(provider, owner, newOwner.publicKey, airdropAmount);

        try {
            await programMethods.legoLend.createMarket(
                newOwner,
                lltv,
                fee,
                usdcMint,
                wbtcMint,
                oracleBase.programId,
                irmBase.programId,
                market,
                loanTokenAccount,
                collateralTokenAccount,
                legoLend
            );
        } catch (err) {
            assert.equal(err.error.errorMessage, errors.addressConstraintViolated);
        }
    });

    it("Market creation fails if max lltv is exceeded", async () => {
        const lltv = new anchor.BN(10e9);
        const fee = new anchor.BN(1e8);
        const market = pda.legoLend.getMarket(usdcMint, wbtcMint, legoLend);
        const loanTokenAccount = pda.legoLend.getVault(market, usdcMint, legoLend);
        const collateralTokenAccount = pda.legoLend.getVault(market, wbtcMint, legoLend);

        try {
            await programMethods.legoLend.createMarket(
                owner,
                lltv,
                fee,
                usdcMint,
                wbtcMint,
                oracleBase.programId,
                irmBase.programId,
                market,
                loanTokenAccount,
                collateralTokenAccount,
                legoLend
            );
        } catch (err) {
            assert.equal((err as AnchorError).error.errorMessage, errors.maxLltvExceeded);
        }
    });

    it("Market creation fails if max fee is exceeded", async () => {
        const lltv = new anchor.BN(9e8);
        const fee = new anchor.BN(10e9);
        const market = pda.legoLend.getMarket(usdcMint, wbtcMint, legoLend);
        const loanTokenAccount = pda.legoLend.getVault(market, usdcMint, legoLend);
        const collateralTokenAccount = pda.legoLend.getVault(market, wbtcMint, legoLend);

        try {
            await programMethods.legoLend.createMarket(
                owner,
                lltv,
                fee,
                usdcMint,
                wbtcMint,
                oracleBase.programId,
                irmBase.programId,
                market,
                loanTokenAccount,
                collateralTokenAccount,
                legoLend
            );
        } catch (err) {
            assert.equal((err as AnchorError).error.errorMessage, errors.maxFeeExceeded);
        }
    });

    it("Market creation succeeds and emits event", async () => {
        const lltv = new anchor.BN(9e8);
        const fee = new anchor.BN(1e8);
        const market = pda.legoLend.getMarket(usdcMint, wbtcMint, legoLend);
        const loanTokenAccount = pda.legoLend.getVault(market, usdcMint, legoLend);
        const collateralTokenAccount = pda.legoLend.getVault(market, wbtcMint, legoLend);

        const createMarketEventListener = legoLend.addEventListener(
            "MarketCreated",
            (eventFields) => {
                assert.equal(eventFields.market.toString(), market.toString());
                assert.equal(eventFields.loanToken.toString(), usdcMint.toString());
                assert.equal(eventFields.collateralToken.toString(), wbtcMint.toString());
                assert.equal(eventFields.oracle.toString(), oracleBase.programId.toString());
                assert.equal(eventFields.irm.toString(), irmBase.programId.toString());
                assert.equal(eventFields.lltv.toNumber(), lltv.toNumber());
                assert.equal(eventFields.fee.toNumber(), fee.toNumber());
            }
        );

        await programMethods.legoLend.createMarket(
            owner,
            lltv,
            fee,
            usdcMint,
            wbtcMint,
            oracleBase.programId,
            irmBase.programId,
            market,
            loanTokenAccount,
            collateralTokenAccount,
            legoLend
        );

        const marketAccount = await legoLend.account.market.fetch(market);

        assert.equal(marketAccount.marketParams.loanToken.toString(), usdcMint.toString());
        assert.equal(marketAccount.marketParams.collateralToken.toString(), wbtcMint.toString());
        assert.equal(marketAccount.marketParams.oracle.toString(), oracleBase.programId.toString());
        assert.equal(marketAccount.marketParams.irm.toString(), irmBase.programId.toString());
        assert.equal(marketAccount.marketParams.lltv.toNumber(), lltv.toNumber());

        assert.equal(marketAccount.fee.toNumber(), fee.toNumber());
        assert.isAbove(marketAccount.lastUpdate.toNumber(), 0);

        assert(
            marketAccount.bump >= bumpRangeInclusive[0] &&
                marketAccount.bump <= bumpRangeInclusive[1]
        );
        assert(
            marketAccount.loanTokenAccountBump >= bumpRangeInclusive[0] &&
                marketAccount.loanTokenAccountBump <= bumpRangeInclusive[1]
        );
        assert(
            marketAccount.collateralTokenAccountBump >= bumpRangeInclusive[0] &&
                marketAccount.collateralTokenAccountBump <= bumpRangeInclusive[1]
        );

        const loanTokenAccountDetails = await getAccount(provider.connection, loanTokenAccount);
        const collateralTokenAccountDetails = await getAccount(
            provider.connection,
            collateralTokenAccount
        );

        assert.isTrue(loanTokenAccountDetails.isInitialized);
        assert.equal(loanTokenAccountDetails.owner.toString(), market.toString());
        assert.isTrue(collateralTokenAccountDetails.isInitialized);
        assert.equal(collateralTokenAccountDetails.owner.toString(), market.toString());

        const delay = 100;
        setTimeout(async () => {
            await legoLend.removeEventListener(createMarketEventListener);
        }, delay);
    });

    it("The same market cannot be created again", async () => {
        const lltv = new anchor.BN(9e8);
        const fee = new anchor.BN(1e8);
        const market = pda.legoLend.getMarket(usdcMint, wbtcMint, legoLend);
        const loanTokenAccount = pda.legoLend.getVault(market, usdcMint, legoLend);
        const collateralTokenAccount = pda.legoLend.getVault(market, wbtcMint, legoLend);

        try {
            await programMethods.legoLend.createMarket(
                owner,
                lltv,
                fee,
                usdcMint,
                wbtcMint,
                oracleBase.programId,
                irmBase.programId,
                market,
                loanTokenAccount,
                collateralTokenAccount,
                legoLend
            );
        } catch {}
    });
});
