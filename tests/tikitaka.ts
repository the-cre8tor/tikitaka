import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Tikitaka } from "../target/types/tikitaka";

describe("tikitaka", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Tikitaka as Program<Tikitaka>;

  it("Setup Game!", async () => {
    const game_keypair = anchor.web3.Keypair.generate();
    const player_one = provider.wallet; // player_one is not a keypair but the wallet of the program's provider.
    const player_two = anchor.web3.Keypair.generate();

    const tx_signature = await program.methods
      .setupGame(player_two.publicKey)
      .accounts({
        game: game_keypair.publicKey,
        playerOne: player_one.publicKey,
      })
      .signers([game_keypair])
      .rpc();

    let game_state = await program.account.game.fetch(game_keypair.publicKey);

    expect(game_state.turn).to.equal(1);
    expect(game_state.players).to.eql([
      player_one.publicKey,
      player_two.publicKey,
    ]);
    expect(game_state.state).to.eql({ active: {} });
    expect(game_state.board).to.eql([
      [null, null, null],
      [null, null, null],
      [null, null, null],
    ]);

    console.log(`Transaction Signature: ${tx_signature}`);
  });
});
