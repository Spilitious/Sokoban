const anchor = require('@project-serum/anchor');
const { Connection, PublicKey, Keypair } = require('@solana/web3.js');
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
  // Generate a new keypair for the transaction signer.
  const user = provider.wallet.payer;
  const groundAccount = Keypair.generate();

  // Define width and height
  const width = 10;
  const height = 10;

  // Initialize the ground account with the specified width and height
  let tx = await program.methods.initialize(width, height).accounts({
    ground: groundAccount.publicKey,
    user: user.publicKey,
    systemProgram: SystemProgram.programId,
  }).signers([groundAccount]).rpc();
  console.log('Initialize Transaction signature', tx);

  // Add a box at position (2, 3)
  tx = await program.methods.addBoxStart(2, 3).accounts({
    ground: groundAccount.publicKey,
    user: user.publicKey,
  }).rpc();
  console.log('Add Box Transaction signature', tx);

  // Fetch the account to check the data
  const accountData = await program.account.ground.fetch(groundAccount.publicKey);
  console.log('Account Data:', accountData.data);

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
  print2DArray(accountData.data, width, height);
})();
