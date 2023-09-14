const anchor = require("@project-serum/anchor");

async function main() {
  const provider = anchor.Provider.local(); 


  const idl = anchor.Program.defaultIdl();
  const program = new anchor.Program(idl, programId, provider);

  const programId = new anchor.web3.PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
  const programPath = "./projects/bounty/programs/bounty/NFT.rs"; 
  const programPath = "./projects/bounty/programs/bounty/Market.rs";
  const programAccount = anchor.web3.Keypair.generate();
  await program.rpc.initialize({
    accounts: {
      programAccount: programAccount.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
    signers: [programAccount],
  });

  console.log("Program deployed to:", programId.toBase58());

  console.log("Deployment completed successfully!");
}

main().catch((err) => {
  console.error(err);
});
