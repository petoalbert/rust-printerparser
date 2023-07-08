import React from "react";
import { open } from "@tauri-apps/api/dialog";
import "./App.css";
import styled from "styled-components";
import { invoke } from "@tauri-apps/api";

const Container = styled.div`
  margin: 0;
  display: flex;
  flex-direction: row;
  text-align: center;
  height: 100vh;
`;

const Sidebar = styled.div`
  height: 100%;
  width: 40vw;
  padding: 20px;
  border-right: 1px solid gray;
`;

const Main = styled.div`
  display: flex;
  flex-direction: column;
  text-align: center;
  align-items: center;
  justify-content: center;
  gap: 10px;

  width: 100%;
  height: 100%;
  padding: 20px;
`;

const Spacer = styled.div<{
  height?: number;
  width?: number;
}>`
  height: ${(props) => props.height ?? 0}px;
  width: ${(props) => props.width ?? 0}px;
`;

const RoundedButton = styled.div`
  border: 1px solid gray;
  border-radius: 4px;
  padding: 4px 8px;
  width: max-content;
  height: max-content;
  cursor: pointer;
  &:hover {
    filter: drop-shadow(0 0 1em #24c8db);
  }
`;

function App() {
  const [path, setPath] = React.useState<string | null>(
    "/Users/bertalankormendy/Developer/rust-printerparser/crates/parserprinter/data/temp/untitled.timeline"
  );
  const [branches, setBranches] = React.useState<string[]>([]);
  const [commits, setCommits] = React.useState<string[]>([]);
  const [currentBranch, setCurrentBranch] = React.useState<string | null>(null);

  React.useEffect(() => {
    if (path == null) {
      setBranches([]);
      return;
    }

    Promise.all([
      invoke("db_list_braches", { dbPath: path }).then((branches) =>
        setBranches(branches as string[])
      ),
      invoke("db_log_checkpoints", { dbPath: path }).then((commits) =>
        setCommits((commits as { hash: string }[]).map((c) => c.hash))
      ),
      invoke("db_get_current_branch", { dbPath: path }).then((currentBranch) =>
        setCurrentBranch(currentBranch as string)
      ),
    ]);
  }, [path]);

  const openDB = React.useCallback(() => {
    open({
      directory: true,
      filters: [
        {
          name: "File",
          extensions: ["timeline"],
        },
      ],
    }).then((selected) => {
      if (Array.isArray(selected)) {
        // user selected multiple directories
        // not supported
        return;
      } else if (selected === null) {
        // user cancelled the selection
      } else {
        // user selected a single directory
        console.log(selected);
        setPath(selected);
      }
    });
  }, []);

  return (
    <Container>
      <Sidebar>
        <Spacer height={50} />
        {currentBranch == null
          ? "No branch selected"
          : `Current branch: ${currentBranch}`}
        <Spacer height={50} />
        <RoundedButton>New branch</RoundedButton>
        {branches.map((branch) => (
          <div key={branch}>{branch}</div>
        ))}
      </Sidebar>
      <Main>
        {path === null ? (
          <div
            style={{
              display: "flex",
              justifyContent: "center",
              alignItems: "center",
              width: "20vw",
              height: "20vw",
              border: "1px solid white",
              borderRadius: 16,
              cursor: "pointer",
            }}
            onClick={openDB}
          >
            Open DB
          </div>
        ) : (
          commits.map((c) => <div key={c}>{c}</div>)
        )}
      </Main>
    </Container>
  );
}

export default App;
