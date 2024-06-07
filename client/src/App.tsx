import { useEffect, useState } from "react";
import Home from "./View/Home";
import { get } from "./service/apiService";
import LoadingGuard from "./components/LoadingGuard";

function App() {
  const [status, setStatus] = useState(undefined);

  useEffect(() => {
    const pinger = async () => {
      setStatus(await get("ping"));
    };
    pinger();

    let interval = setInterval(async () => {
      setStatus(await get("ping"));
    }, 3000);

    return () => {
      clearInterval(interval);
    };
  }, []);

  return (
    <LoadingGuard isLoading={!status}>
      <div className="App">
        <Home />
      </div>
    </LoadingGuard>
  );
}

export default App;
