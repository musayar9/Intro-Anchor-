import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCounterTwo } from "../target/types/anchor_counter_two";
import { expect } from "chai";

describe("anchor-counter-two", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorCounterTwo as Program<AnchorCounterTwo>;

  const counter = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({counter: counter.publicKey}).signers([counter]).rpc();
    
    
    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
    console.log("Your transaction signature", tx);
  });

  it("Incremented the count", async()=>{
    const tx = await program.methods
      .increment()
      .accounts({counter:counter.publicKey, user:provider.wallet.publicKey}).rpc();
      
      const account = await program.account.counter.fetch(counter.publicKey);
      expect(account.count.toNumber()).to.equal(1);

  })

  it("Descremented the count", async()=>{


    
    const tx = await program.methods.decrement().accounts({counter:counter.publicKey, user:provider.wallet.publicKey}).rpc()

    const account = await program.account.counter.fetch(counter.publicKey);
 // Sayaç değerinin 0 olup olmadığını kontrol et
expect(account.count.toNumber()).to.equal(0);
  })
});
