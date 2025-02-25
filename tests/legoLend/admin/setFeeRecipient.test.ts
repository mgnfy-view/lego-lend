import * as anchor from "@coral-xyz/anchor";
import { AnchorError } from "@coral-xyz/anchor";
import { assert } from "chai";

import { errors } from "../../utils/constants";
import { pda, programMethods } from "../../utils/utils";
import { setup } from "../../utils/setup";

describe("LegoLend", () => {
    const { owner, feeRecipient, legoLend } = setup();
    const newFeeRecipient = anchor.web3.Keypair.generate();

    before(async () => {
        await programMethods.legoLend.initialize(owner, feeRecipient.publicKey, legoLend);
    });

    it("Setting new fee recipient fails if caller is not owner", async () => {
        try {
            await programMethods.legoLend.setFeeRecipient(
                newFeeRecipient,
                anchor.web3.PublicKey.default,
                legoLend
            );
        } catch (err) {
            assert.equal(err.error.errorMessage, errors.addressConstraintViolated);
        }
    });

    it("Setting new fee recipient fails if new fee recipient is default pubkey", async () => {
        try {
            await programMethods.legoLend.setFeeRecipient(
                owner,
                anchor.web3.PublicKey.default,
                legoLend
            );
        } catch (err) {
            const errorMessage = (err as AnchorError).error.errorMessage;
            assert.equal(errorMessage, errors.noDefaultPubkey);
        }
    });

    it("Setting new fee recipient succeeds and emits event", async () => {
        const feeRecipientSetEventListener = legoLend.addEventListener(
            "FeeRecipientSet",
            (eventFields) => {
                assert.equal(
                    eventFields.newFeeRecipient.toString(),
                    newFeeRecipient.publicKey.toString()
                );
            }
        );

        await programMethods.legoLend.setFeeRecipient(owner, newFeeRecipient.publicKey, legoLend);

        const platformConfigPubkey = pda.legoLend.getPlatformConfig(legoLend);
        const platformConfig = await legoLend.account.platformConfig.fetch(platformConfigPubkey);

        assert.equal(platformConfig.feeRecipient.toString(), newFeeRecipient.publicKey.toString());

        const delay = 100;
        setTimeout(async () => {
            await legoLend.removeEventListener(feeRecipientSetEventListener);
        }, delay);
    });
});
