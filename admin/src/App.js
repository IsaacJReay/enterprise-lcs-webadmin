import React from "react";
import "./App.css";
import "antd/dist/antd.css";
import { BrowserRouter as Router, Switch } from "react-router-dom";
import Login from "./components/login";
import StatusPage from "./components/status/status";
import WANSetting from "./components/network/WAN";
import WLANSetting from "./components/network/WLAN";
import WirelessSetting from "./components/wireless/wireless";
import DNSSetting from "./components/DNS/dns";
import DNSManagement from "./components/DNS/dns-management/manage-dns";
import PrivateRoute from "./privateRoute";
import PublicRoute from "./publicRoute";
import Storages from "./components/storage/storage";
import Logout from "./components/logout";
import AboutUs from "./components/about";
import StoragesManagement from "./components/storage/storages-management";
import SystemUpdates from "./components/management/updates/system-updates";
import UserAccount from "./components/management/user";
import BackupSetting from "./components/management/backup/backup-restore";
import ResetAll from "./components/management/reset-all";
import TimeSetting from "./components/management/times/time-setting";
import InternalWebsite from "./components/website/internal";
import CustomWebsite from "./components/website/custom-website/main";
import DownloadWebsite from "./components/website/download-website/download";

function App() {
  return (
    <React.Fragment>
      <Router>
        <Switch>
          <PublicRoute exact="true" path="/" component={Login} />
          <PublicRoute exact="true" path="/login" component={Login} />
          <PublicRoute exact="true" path="/logout" component={Logout} />
          <PrivateRoute exact path="/status" component={StatusPage} />
          <PrivateRoute exact path="/network/wan" component={WANSetting} />
          <PrivateRoute exact path="/network/wlan" component={WLANSetting} />
          <PrivateRoute exact path="/wireless" component={WirelessSetting} />
          <PrivateRoute exact path="/dns" component={DNSSetting} />
          <PrivateRoute
            exact
            path="/dns-management/:id"
            component={DNSManagement}
          />
          <PrivateRoute exact path="/storages" component={Storages} />
          <PrivateRoute
            exact
            path="/storages/setting/:id"
            component={StoragesManagement}
          />
          <PrivateRoute exact path="/about-us" component={AboutUs} />
          <PrivateRoute
            exact
            path="/management/system-update"
            component={SystemUpdates}
          />
          <PrivateRoute
            exact
            path="/management/users-account"
            component={UserAccount}
          />
          <PrivateRoute
            exact
            path="/management/backup-restore"
            component={BackupSetting}
          />
          <PrivateRoute exact path="/management/reset" component={ResetAll} />
          <PrivateRoute
            exact
            path="/management/time-setting"
            component={TimeSetting}
          />
          <PrivateRoute
            exact
            path="/website/internal"
            component={InternalWebsite}
          />
          <PrivateRoute
            exact
            path="/website/custom"
            component={CustomWebsite}
          />
          <PrivateRoute
            exact
            path="/website/download"
            component={DownloadWebsite}
          />
        </Switch>
      </Router>
    </React.Fragment>
  );
}

export default App;
