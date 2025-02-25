import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { Connection, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { LegoLend } from "../../target/types/lego_lend";

import { seeds } from "./constants";
import { IrmBase } from "../../target/types/irm_base";
import { OracleBase } from "../../target/types/oracle_base";

async function createSplTokenMint(
    connection: Connection,
    owner: anchor.web3.Keypair,
    decimals: number
) {
    return await spl.createMint(connection, owner, owner.publicKey, owner.publicKey, decimals);
}

async function transfer(
    provider: anchor.AnchorProvider,
    from: anchor.web3.Keypair,
    to: anchor.web3.PublicKey,
    amount: number
) {
    const transaction = new Transaction().add(
        SystemProgram.transfer({
            fromPubkey: from.publicKey,
            toPubkey: to,
            lamports: amount,
        })
    );
    await sendAndConfirmTransaction(provider.connection, transaction, [from]);
}

const pda = {
    irmBase: {
        getBorrowRate(program: anchor.Program<IrmBase>) {
            return anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from(seeds.borrowRate)],
                program.programId
            )[0];
        },
    },
    legoLend: {
        getPlatformConfig(program: anchor.Program<LegoLend>) {
            return anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from(seeds.platformConfig)],
                program.programId
            )[0];
        },
        getMarket(
            loanTokenMint: anchor.web3.PublicKey,
            collateralTokenMint: anchor.web3.PublicKey,
            program: anchor.Program<LegoLend>
        ) {
            return anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from(seeds.market),
                    loanTokenMint.toBuffer(),
                    collateralTokenMint.toBuffer(),
                ],
                program.programId
            )[0];
        },
        getVault(
            market: anchor.web3.PublicKey,
            tokenMint: anchor.web3.PublicKey,
            program: anchor.Program<LegoLend>
        ) {
            return anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from(seeds.vault), market.toBuffer(), tokenMint.toBuffer()],
                program.programId
            )[0];
        },
    },
    oracleBase: {
        getPrice(program: anchor.Program<OracleBase>) {
            return anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from(seeds.price)],
                program.programId
            )[0];
        },
    },
};

const programMethods = {
    irmBase: {
        async initialize(
            owner: anchor.web3.Keypair,
            borrowRate: anchor.BN,
            program: anchor.Program<IrmBase>
        ) {
            await program.methods
                .initialize(borrowRate)
                .accounts({
                    owner: owner.publicKey,
                    borrowRate: pda.irmBase.getBorrowRate(program),
                })
                .signers([owner])
                .rpc();
        },
        async update(program: anchor.Program<IrmBase>) {
            await program.methods
                .update(
                    new anchor.BN(0),
                    new anchor.BN(0),
                    new anchor.BN(0),
                    new anchor.BN(0),
                    new anchor.BN(0),
                    new anchor.BN(0)
                )
                .accounts({
                    borrowRate: pda.irmBase.getBorrowRate(program),
                    supplementalIrmAccount1: anchor.web3.Keypair.generate().publicKey,
                    supplementalIrmAccount2: anchor.web3.Keypair.generate().publicKey,
                })
                .rpc();
        },
    },
    legoLend: {
        async initialize(
            owner: anchor.web3.Keypair,
            feeRecipient: anchor.web3.PublicKey,
            program: anchor.Program<LegoLend>
        ) {
            await program.methods
                .initialize()
                .accounts({
                    owner: owner.publicKey,
                    feeRecipient,
                    platformConfig: pda.legoLend.getPlatformConfig(program),
                })
                .signers([owner])
                .rpc();
        },
        async setFeeRecipient(
            owner: anchor.web3.Keypair,
            newFeeRecipient: anchor.web3.PublicKey,
            program: anchor.Program<LegoLend>
        ) {
            await program.methods
                .setFeeRecipient()
                .accounts({
                    owner: owner.publicKey,
                    newFeeRecipient,
                    platformConfig: pda.legoLend.getPlatformConfig(program),
                })
                .signers([owner])
                .rpc();
        },
        async transferOwnership(
            owner: anchor.web3.Keypair,
            newOwner: anchor.web3.Keypair,
            program: anchor.Program<LegoLend>
        ) {
            await program.methods
                .transferOwnership()
                .accounts({
                    owner: owner.publicKey,
                    newOwner: newOwner.publicKey,
                    platformConfig: pda.legoLend.getPlatformConfig(program),
                })
                .signers([owner, newOwner])
                .rpc();
        },
        async createMarket(
            owner: anchor.web3.Keypair,
            lltv: anchor.BN,
            fee: anchor.BN,
            loanToken: anchor.web3.PublicKey,
            collateralToken: anchor.web3.PublicKey,
            oracle: anchor.web3.PublicKey,
            irm: anchor.web3.PublicKey,
            market: anchor.web3.PublicKey,
            loanTokenAccount: anchor.web3.PublicKey,
            collateralTokenAccount: anchor.web3.PublicKey,
            program: anchor.Program<LegoLend>
        ) {
            await program.methods
                .createMarket(lltv, fee)
                .accounts({
                    creator: owner.publicKey,
                    platformConfig: pda.legoLend.getPlatformConfig(program),
                    loanToken,
                    collateralToken,
                    oracle,
                    irm,
                    market,
                    loanTokenAccount,
                    collateralTokenAccount,
                    tokenProgram: spl.TOKEN_PROGRAM_ID,
                })
                .signers([owner])
                .rpc();
        },
    },
    oracleBase: {
        async initialize(
            owner: anchor.web3.Keypair,
            price: anchor.BN,
            program: anchor.Program<OracleBase>
        ) {
            await program.methods
                .initialize(price)
                .accounts({
                    owner: owner.publicKey,
                    price: pda.oracleBase.getPrice(program),
                })
                .signers([owner])
                .rpc();
        },
        async update(program: anchor.Program<OracleBase>) {
            await program.methods
                .update()
                .accounts({
                    price: pda.oracleBase.getPrice(program),
                    supplementalOracleAccount1: anchor.web3.Keypair.generate().publicKey,
                    supplementalOracleAccount2: anchor.web3.Keypair.generate().publicKey,
                })
                .rpc();
        },
    },
};

export { createSplTokenMint, transfer, pda, programMethods };
