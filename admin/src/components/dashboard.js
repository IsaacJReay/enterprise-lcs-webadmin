import React from "react";
import { Layout } from "antd";
import NavBar from "./layouts/navbar";
import SideNavBar from "./layouts/side-navbar";

const { Content } = Layout;

const Dashboard = () => {
  return (
    <React.Fragment>
      <Layout style={{ minHeight: "100vh" }}>
        <NavBar />
        <Layout>
          <SideNavBar />
          <Content>
            <h1>hello world</h1>
          </Content>
        </Layout>
      </Layout>
    </React.Fragment>
  );
};

export default Dashboard;
