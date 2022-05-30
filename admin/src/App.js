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
import PageNotFound from "./components/404";

function App() {
  return (
    <React.Fragment>
      <Router>
        <Switch>
          <PublicRoute exact={true} path="/" component={Login} />
          <PublicRoute exact={true} path="/login" component={Login} />
          <PublicRoute exact={true} path="/logout" component={Logout} />
          <PrivateRoute exact={true} path="/status" component={StatusPage} />
          <PrivateRoute
            exact={true}
            path="/network/wan"
            component={WANSetting}
          />
          <PrivateRoute
            exact={true}
            path="/network/wlan"
            component={WLANSetting}
          />
          <PrivateRoute
            exact={true}
            path="/wireless"
            component={WirelessSetting}
          />
          <PrivateRoute exact="true" path="/dns" component={DNSSetting} />
          <PrivateRoute
            exact={true}
            path="/dns-management/:zones/:slug"
            component={DNSManagement}
          />
          <PrivateRoute exact path="/storages" component={Storages} />
          <PrivateRoute
            exact={true}
            path="/storages/setting/:id"
            component={StoragesManagement}
          />
          <PrivateRoute exact path="/about-us" component={AboutUs} />
          <PrivateRoute
            exact={true}
            path="/management/system-update"
            component={SystemUpdates}
          />
          <PrivateRoute
            exact={true}
            path="/management/users-account"
            component={UserAccount}
          />
          <PrivateRoute
            exact={true}
            path="/management/backup-restore"
            component={BackupSetting}
          />
          <PrivateRoute exact path="/management/reset" component={ResetAll} />
          <PrivateRoute
            exact={true}
            path="/management/time-setting"
            component={TimeSetting}
          />
          <PrivateRoute
            exact={true}
            path="/website/internal"
            component={InternalWebsite}
          />
          <PrivateRoute
            exact={true}
            path="/website/custom"
            component={CustomWebsite}
          />
          <PrivateRoute
            exact={true}
            path="/website/download"
            component={DownloadWebsite}
          />
          <PublicRoute component={PageNotFound} />
        </Switch>
      </Router>
    </React.Fragment>
  );
}

export default App;
