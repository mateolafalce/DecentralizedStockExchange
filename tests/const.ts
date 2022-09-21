import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;