import React from "react";
import { Route, Redirect } from "react-router-dom";
import { Layout } from "antd";
import SideNavBar from "./components/layouts/side-navbar";

const { Content } = Layout;
let token = localStorage.getItem("token");

const PrivateRoute = ({ component: Component, ...rest }) => {
  const isLogin = () => {
    if (token) {
      if (token.reason === "token-timeout") {
        window.location.replace("/logout");
      }
      return true;
    } else {
      return false;
    }
  };
  return (
    <>
      <Route
        {...rest}
        render={(props) =>
          isLogin() ? (
            <Layout style={{ minHeight: "100vh" }}>
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
    </>
  );
};

export default PrivateRoute;
