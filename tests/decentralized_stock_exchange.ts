import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DecentralizedStockExchange } from "../target/types/decentralized_stock_exchange";

describe("decentralized_stock_exchange", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DecentralizedStockExchange as Program<DecentralizedStockExchange>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
