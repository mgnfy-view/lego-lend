import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

import { pda, programMethods } from "../utils/utils";
import { setup } from "../utils/setup";

describe("IrmBase", () => {
    const { owner, irmBase } = setup();
    const borrowRate = new anchor.BN(1e9);

    before(async () => {
        await programMethods.irmBase.initialize(owner, borrowRate, irmBase);
    });

    it("Successfully updates borrow rate", async () => {
        const updateEventListener = irmBase.addEventListener("BorrowRateUpdated", (eventFields) => {
            assert.equal(eventFields.borrowRate.toNumber(), borrowRate.toNumber());
        });

        await programMethods.irmBase.update(irmBase);

        const borrowRatePdaPubkey = pda.irmBase.getBorrowRate(irmBase);
        const borrowRateAccount = await irmBase.account.borrowRate.fetch(borrowRatePdaPubkey);

        assert.equal(borrowRateAccount.borrowRate.toNumber(), borrowRate.toNumber());

        const delay = 100;
        setTimeout(async () => {
            await irmBase.removeEventListener(updateEventListener);
        }, delay);
    });
});
