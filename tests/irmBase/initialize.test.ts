import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

import { bumpRangeInclusive, errors } from "../utils/constants";
import { pda, programMethods } from "../utils/utils";
import { setup } from "../utils/setup";

describe("IrmBase", () => {
    const { owner, irmBase } = setup();
    const borrowRate = new anchor.BN(1e9);

    it("Initialization fails if price passed is 0", async () => {
        try {
            await programMethods.irmBase.initialize(owner, new anchor.BN(0), irmBase);
        } catch (err) {
            assert.equal((err as anchor.AnchorError).error.errorMessage, errors.valueZero);
        }
    });

    it("Initialization succeeds and emits event", async () => {
        const initializeEventListener = irmBase.addEventListener("Initialized", (eventFields) => {
            assert.equal(eventFields.borrowRate.toNumber(), borrowRate.toNumber());
        });

        await programMethods.irmBase.initialize(owner, borrowRate, irmBase);

        const borrowRatePdaPubkey = pda.irmBase.getBorrowRate(irmBase);
        const borrowRateAccount = await irmBase.account.borrowRate.fetch(borrowRatePdaPubkey);

        assert.equal(borrowRateAccount.borrowRate.toNumber(), borrowRate.toNumber());
        assert(
            borrowRateAccount.bump >= bumpRangeInclusive[0] &&
                borrowRateAccount.bump <= bumpRangeInclusive[1]
        );

        const delay = 100;
        setTimeout(async () => {
            await irmBase.removeEventListener(initializeEventListener);
        }, delay);
    });

    it("Initialization fails if done twice", async () => {
        try {
            await programMethods.irmBase.initialize(owner, borrowRate, irmBase);
        } catch {}
    });
});
