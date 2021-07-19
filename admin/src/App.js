import React from "react";
import "./App.css";
import "antd/dist/antd.css";
import { BrowserRouter as Router, Switch } from "react-router-dom";
import Login from "./components/login";
import StatusPage from "./components/status/status";
import WANSetting from "./components/network/WAN";
import WLANSetting from "./components/network/WLAN";
import WirelessSetting from "./components/wireless/wireless";
import DHCPSetting from "./components/DHPC/dhcp";
import DNSSetting from "./components/DNS/dns";
import DNSManagement from "./components/DNS/manage-dns";
import PrivateRoute from "./privateRoute";
import PublicRoute from "./publicRoute";
import Storages from "./components/storage/storage";
import Logout from "./components/logout";
import AboutUs from "./components/about";
import LocalStorage from "./components/storage/local-storage";

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
          <PrivateRoute exact path="/dhcp" component={DHCPSetting} />
          <PrivateRoute exact path="/dns" component={DNSSetting} />
          <PrivateRoute
            exact
            path="/dns-management"
            component={DNSManagement}
          />
          <PrivateRoute exact path="/storages" component={Storages} />
          <PrivateRoute exact path="/storages/local" component={LocalStorage} />
          <PrivateRoute exact path="/about-us" component={AboutUs} />
        </Switch>
      </Router>
    </React.Fragment>
  );
}

export default App;
