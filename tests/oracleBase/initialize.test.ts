import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

import { bumpRangeInclusive, errors } from "../utils/constants";
import { pda, programMethods } from "../utils/utils";
import { setup } from "../utils/setup";

describe("OracleBase", () => {
    const { owner, oracleBase } = setup();
    const price = new anchor.BN(1e9);

    it("Initialization fails if price passed is 0", async () => {
        try {
            await programMethods.oracleBase.initialize(owner, new anchor.BN(0), oracleBase);
        } catch (err) {
            assert.equal((err as anchor.AnchorError).error.errorMessage, errors.valueZero);
        }
    });

    it("Initialization succeeds and emits event", async () => {
        const initializeEventListener = oracleBase.addEventListener(
            "Initialized",
            (eventFields) => {
                assert.equal(eventFields.price.toNumber(), price.toNumber());
            }
        );

        await programMethods.oracleBase.initialize(owner, price, oracleBase);

        const pricePdaPubkey = pda.oracleBase.getPrice(oracleBase);
        const priceAccount = await oracleBase.account.price.fetch(pricePdaPubkey);

        assert.equal(priceAccount.price.toNumber(), price.toNumber());
        assert(
            priceAccount.bump >= bumpRangeInclusive[0] && priceAccount.bump <= bumpRangeInclusive[1]
        );

        const delay = 100;
        setTimeout(async () => {
            await oracleBase.removeEventListener(initializeEventListener);
        }, delay);
    });

    it("Initialization fails if done twice", async () => {
        try {
            await programMethods.oracleBase.initialize(owner, price, oracleBase);
        } catch {}
    });
});
