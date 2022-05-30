import React from "react";
import { Route, Redirect } from "react-router-dom";
import { Layout, Modal } from "antd";
import SideNavBar from "./components/layouts/side-navbar";
import jwt_decode from "jwt-decode";
import { ExclamationCircleOutlined } from "@ant-design/icons";
import Cookies from "js-cookie";

const { Content } = Layout;

const PrivateRoute = ({ component: Component, ...rest }) => {
  // let token = localStorage.getItem("token");

  let token = Cookies.get("token");
  var decoded = jwt_decode(token);
  const isNow = Math.floor(new Date().getTime() / 1000.0);
  
  function onLogout() {
    return window.location.replace("/logout");
  }

  const isLogin = () => {
    if (token) {
      if (decoded.exp <= isNow) {
        return Modal.error({
          title: "Your permission is time out!",
          icon: <ExclamationCircleOutlined />,
          content:
            "You cannot do anything! Re-login if you want to have more permission!",
          okText: "Logout",
          onOk: onLogout,
        });
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
