import { initializeKeypair } from "./init";
import * as web3 from "@solana/web3.js";
import * as game from "./game";
import { select } from "@inquirer/prompts";
import { onExit } from "signal-exit";

onExit((code, signal) => {
  console.log("process exited!", code, signal);
  process.exit(code as number);
});

const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed");
// const connection = new web3.Connection(web3.clusterApiUrl("devnet"),"finalized");

const programId = new web3.PublicKey(
  "4kUz8auT49b6s1mUiPTdbnmwhPiVLjpaGkVmh81Zjygj"
);

const main = async () => {
  const pair = await initializeKeypair(connection);
  const one_game = web3.Keypair.generate();
  console.log("account : ", pair.publicKey.toBase58());
  console.log("one game address : ", one_game.publicKey.toBase58());

  const transaction = new web3.Transaction().add(
    new web3.TransactionInstruction({
      keys: [
        {
          pubkey: one_game.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: pair.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: web3.SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      programId,
      data: Buffer.from([0]),
    })
  );

  const signature = await web3.sendAndConfirmTransaction(
    connection,
    transaction,
    [pair, one_game]
  );

  console.log("Signature : ", signature);

  const info = await connection.getTransaction(signature, {
    commitment: "confirmed",
    maxSupportedTransactionVersion: 2,
  });
  console.log("transaction info : ", info?.meta?.logMessages);

  const gameData = await connection.getAccountInfo(one_game.publicKey);
  console.log("game data : ", gameData?.data);

  if (gameData?.data) {
    const gameDataParsed = game.ParseGame(gameData.data);
    console.log("game data parsed : ", gameDataParsed);
  }

  while (true) {
    try {
      const answer = await select({
        message: "Enter your direction: ",
        choices: [
          {
            name: "left",
            value: 1,
            description: "move left",
          },
          {
            name: "right",
            value: 2,
            description: "move right",
          },
          {
            name: "up",
            value: 3,
            description: "move up",
          },
          {
            name: "down",
            value: 4,
            description: "move down",
          },
        ],
      });
      console.log(answer);
    } catch (error) {
      console.log(error);
    }
  }
};

main()
  .then(() => {
    console.log("Finished successfully");
    process.exit(0);
  })
  .catch((error) => {
    console.log(error);
    process.exit(1);
  });
