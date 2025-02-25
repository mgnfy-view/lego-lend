import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

import { errors } from "../../utils/constants";
import { pda, programMethods } from "../../utils/utils";
import { setup } from "../../utils/setup";

describe("LegoLend", () => {
    const { owner, feeRecipient, legoLend } = setup();
    const newOwner = anchor.web3.Keypair.generate();

    before(async () => {
        await programMethods.legoLend.initialize(owner, feeRecipient.publicKey, legoLend);
    });

    it("Setting new owner fails if caller is not owner", async () => {
        try {
            await programMethods.legoLend.transferOwnership(newOwner, newOwner, legoLend);
        } catch (err) {
            assert.equal(err.error.errorMessage, errors.addressConstraintViolated);
        }
    });

    it("Setting new owner succeeds and emits event", async () => {
        const ownershipTransferredEventListener = legoLend.addEventListener(
            "OwnershipTransferred",
            (eventFields) => {
                assert.equal(eventFields.newOwner.toString(), newOwner.publicKey.toString());
            }
        );

        await programMethods.legoLend.transferOwnership(owner, newOwner, legoLend);

        const platformConfigPubkey = pda.legoLend.getPlatformConfig(legoLend);
        const platformConfig = await legoLend.account.platformConfig.fetch(platformConfigPubkey);

        assert.equal(platformConfig.owner.toString(), newOwner.publicKey.toString());

        const delay = 100;
        setTimeout(async () => {
            await legoLend.removeEventListener(ownershipTransferredEventListener);
        }, delay);
    });
});
