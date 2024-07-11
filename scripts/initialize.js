const anchor = require('@project-serum/anchor');
const { Connection, PublicKey, Keypair, SystemProgram } = require('@solana/web3.js');
const fs = require('fs');

// Set the environment variable ANCHOR_WALLET
process.env.ANCHOR_WALLET = process.env.HOME + '/.config/solana/id.json';

// Configure the local cluster.
const provider = anchor.AnchorProvider.local();
anchor.setProvider(provider);

// Address of the deployed program.
const programId = new PublicKey('FYjcKSeCtxwWi161uNjmN8cs2ykVtA2YWdpnsyAWjuHK');

// Load the IDL.
const idl = JSON.parse(fs.readFileSync('target/idl/projet_final.json', 'utf8'));

// Create a new program client.
const program = new anchor.Program(idl, programId, provider);


(async () => {
  try {
  // Generate a new keypair for the transaction signer.
  const user = provider.wallet.payer;
  const gameAccount= Keypair.generate();

  // Define width and height
  const width = 3;
  const height = 3;
  const ground = new Array(width * height).fill(0);

  // Initialize the ground account with the specified width and height
  let tx = await program.methods.initialize(width, height, ground).accounts({
    game: gameAccount.publicKey,
    user: user.publicKey,
    systemProgram: SystemProgram.programId,
  }).signers([gameAccount]).rpc();
  console.log('Initialize Transaction signature', tx);

  // Fetch the account to check the data
  const accountData = await program.account.ground.fetch(gameAccount.publicKey);
  //console.log('Account Data:', accountData.data);

  // Function to print the data in 2D format
  function print2DArray(data, width, height) {
    for (let y = 0; y < height; y++) {
      let row = '';
      for (let x = 0; x < width; x++) {
        row += data[y * width + x] + ' ';
      }
      console.log(row);
    }
  }

  // Print the 2D array
  print2DArray(accountData.map_data, width, height);
} catch (error) {
  console.error("Error:", error);
}
})();