import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

import { pda, programMethods } from "../utils/utils";
import { setup } from "../utils/setup";

describe("OracleBase", () => {
    const { owner, oracleBase } = setup();
    const price = new anchor.BN(1e9);

    before(async () => {
        await programMethods.oracleBase.initialize(owner, price, oracleBase);
    });

    it("Successfully updates price", async () => {
        const updateEventListener = oracleBase.addEventListener("PriceUpdated", (eventFields) => {
            assert.equal(eventFields.price.toNumber(), price.toNumber());
        });

        await programMethods.oracleBase.update(oracleBase);

        const pricePdaPubkey = pda.oracleBase.getPrice(oracleBase);
        const priceAccount = await oracleBase.account.price.fetch(pricePdaPubkey);

        assert.equal(priceAccount.price.toNumber(), price.toNumber());

        const delay = 100;
        setTimeout(async () => {
            await oracleBase.removeEventListener(updateEventListener);
        }, delay);
    });
});
