import { AdminWebsocket } from "@holochain/client";

AdminWebsocket.connect().then(async (adminWs) => {
  const apps = await adminWs.listApps({});

  const appListDiv = document.getElementById("app-list")!;

  // Simply list all installed apps

  if (apps.length === 0)
    appListDiv.innerHTML = `<span>There are no apps installed yet</span>`;
  else {
    appListDiv.innerHTML = `<ul>${apps.map((app) => `<li>${app.installed_app_id}</li>`)}</ul>`;
  }
});
