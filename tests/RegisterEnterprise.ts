import * as anchor from "@project-serum/anchor";
import { DecentralizedStockExchange } from "../target/types/decentralized_stock_exchange";
import { wallet } from "./const";
import { PublicKey } from '@solana/web3.js';

describe("Creating PDA", () => {
  const program = anchor.workspace.DecentralizedStockExchange as anchor.Program<DecentralizedStockExchange>;
    it("Created", async () => {
      const [Enterprise, _bump1] = await PublicKey
      .findProgramAddress(
        [
          wallet.publicKey.toBuffer(),
        ],
        program.programId
      );
      const [Offers, _bump2] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("Offers"),
          wallet.publicKey.toBuffer(),
        ],
        program.programId
      );
      const tx = await program.methods.registerEnterprise(
        new anchor.BN(5000),
        new anchor.BN(1090292400)
      )
      .accounts({
          enterprise: Enterprise,
          offers: Offers,
          user: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
      console.log("---------------------------------------------")
      console.log("PDA: ", Enterprise.toBase58());
      console.log("---------------------------------------------") 
      console.log("Your transaction signature", tx);
    });
})