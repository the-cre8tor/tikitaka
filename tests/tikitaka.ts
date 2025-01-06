import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Tikitaka } from "../target/types/tikitaka";
import { Keypair } from "@solana/web3.js";

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

  it("player one wins", async () => {
    const game_keypair = Keypair.generate();
    const player_one = provider.wallet;
    const player_two = Keypair.generate();

    const tx_signature = await program.methods
      .setupGame(player_two.publicKey)
      .accounts({
        game: game_keypair.publicKey,
        playerOne: player_one.publicKey,
      })
      .signers([game_keypair])
      .rpc();

    let game_state = await program.account.game.fetch(game_keypair.publicKey);

    // assert - project initialization
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

    // assert - move confirmation
    await program.methods
      .play({ row: 0, column: 0 })
      .accounts({
        player: player_one.publicKey,
        game: game_keypair.publicKey,
      })
      .rpc();

    const game_move_state = await program.account.game.fetch(
      game_keypair.publicKey
    );

    expect(game_move_state.turn).to.equal(2);
    expect(game_move_state.state).to.eql({ active: {} });
    expect(game_move_state.board).to.eql([
      [{ x: {} }, null, null],
      [null, null, null],
      [null, null, null],
    ]);

    console.log(`Transaction Signature: ${tx_signature}`);
  });
});
