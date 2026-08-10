defmodule Workbench.GuestAgentTest do
  use ExUnit.Case, async: true

  alias Workbench.GuestAgent

  test "accepts Pi's canonical last assistant text response" do
    assert {:ok, "done"} =
             GuestAgent.extract_last_assistant_text(%{"text" => "  done\n"})
  end

  test "rejects missing or blank assistant text" do
    assert {:error, :empty_agent_reply} =
             GuestAgent.extract_last_assistant_text(%{"text" => "  "})

    assert {:error, :empty_agent_reply} =
             GuestAgent.extract_last_assistant_text(%{"text" => nil})

    assert {:error, :empty_agent_reply} = GuestAgent.extract_last_assistant_text(%{})
  end
end
