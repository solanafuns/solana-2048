import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Anchor2048 } from "../target/types/anchor_2048";

describe("anchor-2048", () => {
  const game = web3.Keypair.generate();
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const directionText = ["up", "down", "left", "right"];

  const program = anchor.workspace.Anchor2048 as Program<Anchor2048>;
  const methods = program.methods;

  const waitAndPrint = async (
    description: string,
    tx: Promise<string>,
    address: web3.PublicKey
  ) => {
    console.log(description);
    const txid = await tx;
    console.info(" txid => [%s]", txid);

    const game: any = await program.account.game.fetch(address);
    console.info(
      "steps : %s , ended: %s ,max : %s ",
      game.steps.toString(),
      game.ended.toString(),
      game.max.toString()
    );
    for (let line of game.board) {
      console.log(line.toString());
    }
  };

  it("let's born a game!", async () => {
    const tx = await methods
      .initialize()
      .accounts({
        newGame: game.publicKey,
        signer: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([game])
      .rpc();

    program.provider.connection.confirmTransaction(tx, "singleGossip");
    console.log(tx);
  });

  it("let's move the board!", async () => {
    const tx = await methods
      .moveBoard(0)
      .accounts({
        game: game.publicKey,
      })
      .rpc();
    program.provider.connection.confirmTransaction(tx, "singleGossip");
    console.log(tx);
  });

  it("let's move 100 steps !", async () => {
    for (let i = 0; i < 100; i++) {
      console.log("\n-------\n");
      let direction = Math.floor(Math.random() * 10) % 4;
      await waitAndPrint(
        "move as description: " + direction + " " + directionText[direction],
        methods
          .moveBoard(direction)
          .accounts({
            game: game.publicKey,
          })
          .rpc(),
        game.publicKey
      ).catch((err) => {
        console.log(err);
      });
    }
  });
});
