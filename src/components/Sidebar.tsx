import "./Sidebar.css";

function Sidebar() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <div className="sidebar">
      <h1 className="text-3xl font-bold underline">
        This is Sidebar
      </h1>
    </div>
  );
}

export default Sidebar;
