import * as anchor from "@coral-xyz/anchor";
import { LegoLend } from "../../target/types/lego_lend";
import { IrmBase } from "../../target/types/irm_base";
import { OracleBase } from "../../target/types/oracle_base";

export function setup() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const owner = (provider.wallet as anchor.Wallet).payer;
    const feeRecipient = anchor.web3.Keypair.generate();

    const legoLend = anchor.workspace.LegoLend as anchor.Program<LegoLend>;
    const irmBase = anchor.workspace.IrmBase as anchor.Program<IrmBase>;
    const oracleBase = anchor.workspace.OracleBase as anchor.Program<OracleBase>;

    return {
        provider,
        owner,
        feeRecipient,
        legoLend,
        irmBase,
        oracleBase,
    };
}
