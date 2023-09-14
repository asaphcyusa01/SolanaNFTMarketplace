import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bounty } from "../target/types/bounty";

describe("Bounty Program", () => {
  anchor.setProvider(anchor.Provider.env());

  let program: Program<Bounty>;

  before(async () => {
    program = anchor.workspace.Bounty as Program<Bounty>;
  });

  it("Should initialize correctly", async () => {
    const tx = await program.rpc.initialize();
    const state = await program.state.fetch();

    assert.ok(state.isInitialized === true);
  });

});
