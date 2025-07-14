import { PublicKey } from "@solana/web3.js";

(async () => {
  const programId = new PublicKey("zKDU6TVerMNPx4eVrFZutFBb4CWD6DzauTtYohjJobW");

  // Your seeds, like Buffer.from("my_seed")
  const [treasurydashpda] = await PublicKey.findProgramAddress(
    [
      Buffer.from("treasurydash"), 
      // Buffer.from("another_seed"), 
      // new PublicKey("something_else").toBuffer()
    ],
    programId
  );

  console.log("PDA:", treasurydashpda.toBase58());
})();