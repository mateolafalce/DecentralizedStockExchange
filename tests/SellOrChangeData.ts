import * as anchor from "@project-serum/anchor";
import { DecentralizedStockExchange } from "../target/types/decentralized_stock_exchange";
import { wallet } from "./const";
import { PublicKey } from '@solana/web3.js';

describe("Creating PDA", () => {
  const program = anchor.workspace.DecentralizedStockExchange as anchor.Program<DecentralizedStockExchange>;
    it("Created", async () => {
      const [Enterprise, _bump] = await PublicKey
      .findProgramAddress(
        [
          wallet.publicKey.toBuffer(),
        ],
        program.programId
      )
      const tx = await program.methods.registerEnterprise()
      .accounts({
          enterprise: Enterprise,
          user: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
      console.log("---------------------------------------------")
      console.log("PDA: ", Enterprise.toBase58());
      console.log("---------------------------------------------") 
      console.log("Your transaction signature", tx);
    });
})