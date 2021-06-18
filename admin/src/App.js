import React from "react";
import "./App.css";
import "antd/dist/antd.css";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import Login from "./components/login";
import StatusPage from "./components/status/status";
import WANSetting from "./components/network/WAN";
import WLANSetting from "./components/network/WLAN";
import WirelessSetting from "./components/wireless/wireless";
import DHCPSetting from "./components/DHPC/dhcp";
import DNSSetting from "./components/DNS/dns";
import DNSManagement from "./components/DNS/manage-dns";

function App() {
  return (
    <React.Fragment>
      <Router>
        <Switch>
          <Route exact path="/" component={Login} />
          <Route exact path="/login" component={Login} />
          <Route exact path="/status" component={StatusPage} />
          <Route exact path="/network/wan" component={WANSetting} />
          <Route exact path="/network/wlan" component={WLANSetting} />
          <Route exact path="/wireless" component={WirelessSetting} />
          <Route exact path="/dhcp" component={DHCPSetting} />
          <Route exact path="/dns" component={DNSSetting} />
          <Route exact path="/dns-management" component={DNSManagement} />
        </Switch>
      </Router>
    </React.Fragment>
  );
}

export default App;
