import React from "react";
import { Route, Redirect, useHistory } from "react-router-dom";
import { Layout } from "antd";
import NavBar from "./components/layouts/navbar";
import SideNavBar from "./components/layouts/side-navbar";
import jwt from "jsonwebtoken";

const { Content } = Layout;

let token = localStorage.getItem("token");

const PrivateRoute = ({ component: Component, ...rest }) => {
  let history = useHistory();

  const isLogin = () => {
    const exp = jwt.decode(token);
    const expirationTime = exp * 1000 - 60000;
    if (token) {
      return true;
    } else if (Date.now() >= expirationTime) {
      localStorage.clear();
      history.push("/login");
    } else {
      return false;
    }
  };
  return (
    <Route
      {...rest}
      render={(props) =>
        isLogin() ? (
          <Layout style={{ minHeight: "100vh" }}>
            <NavBar />
            <Layout>
              <SideNavBar />
              <Content className="content-padding">
                <Component {...props} />
              </Content>
            </Layout>
          </Layout>
        ) : (
          <Redirect to="/login" />
        )
      }
    />
  );
};

export default PrivateRoute;
