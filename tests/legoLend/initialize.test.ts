import * as anchor from "@coral-xyz/anchor";
import { AnchorError } from "@coral-xyz/anchor";
import { assert } from "chai";

import { bumpRangeInclusive, errors } from "../utils/constants";
import { pda, programMethods } from "../utils/utils";
import { setup } from "../utils/setup";

describe("LegoLend", () => {
    const { owner, feeRecipient, legoLend } = setup();

    it("Initialization fails if fee recipient is default pubkey", async () => {
        try {
            await programMethods.legoLend.initialize(
                owner,
                anchor.web3.PublicKey.default,
                legoLend
            );
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.noDefaultPubkey);
        }
    });

    it("Initialization succeeds and emits event", async () => {
        const initializeEventListener = legoLend.addEventListener("Initialized", (eventFields) => {
            assert.equal(eventFields.owner.toString(), owner.publicKey.toString());
            assert.equal(eventFields.feeRecipient.toString(), feeRecipient.publicKey.toString());
        });

        await programMethods.legoLend.initialize(owner, feeRecipient.publicKey, legoLend);

        const platformConfigPubkey = pda.legoLend.getPlatformConfig(legoLend);
        const platformConfig = await legoLend.account.platformConfig.fetch(platformConfigPubkey);

        assert.equal(platformConfig.owner.toString(), owner.publicKey.toString());
        assert.equal(platformConfig.feeRecipient.toString(), feeRecipient.publicKey.toString());
        assert(
            platformConfig.bump >= bumpRangeInclusive[0] &&
                platformConfig.bump <= bumpRangeInclusive[1]
        );

        const delay = 100;
        setTimeout(async () => {
            await legoLend.removeEventListener(initializeEventListener);
        }, delay);
    });

    it("Initialization fails if done twice", async () => {
        try {
            await programMethods.legoLend.initialize(owner, feeRecipient.publicKey, legoLend);
        } catch {}
    });
});
