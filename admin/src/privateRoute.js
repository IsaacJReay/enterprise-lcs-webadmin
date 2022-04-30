import React, { useEffect } from "react";
import { Route, Redirect } from "react-router-dom";
import { Layout } from "antd";
import SideNavBar from "./components/layouts/side-navbar";
import jwt_decode from "jwt-decode";

const { Content } = Layout;

const PrivateRoute = ({ component: Component, ...rest }) => {
  let token = localStorage.getItem("token");

  const isLogin = () => {
    if (token) {
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
