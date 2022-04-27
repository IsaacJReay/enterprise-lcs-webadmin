import React, { useState } from "react";
import { Input, Button, Form, Col, Row, message, notification } from "antd";
import axios from "axios";
import image from "../assets/images/login2.png";
import icon from "../assets/images/icons/koompi-black.png";

const baseUrl = process.env.REACT_APP_API_URL;

const Login = () => {
  const [, setLoading] = useState(false);

  const onSubmit = async (data) => {
    const adminLogin = {
      username: data.username,
      password: data.password,
    };
    await axios
      .post(`${baseUrl}/user/login`, adminLogin)
      .then((res) => {
        if ((res.statusCode = 200)) {
          localStorage.setItem("token", res.data.token);
          setLoading(true);
          message.success("Successful!");
          window.location.replace("/status");
        } else {
          setLoading(true);
          message.error("Invalide username or password ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error(err.response.data.reason);
      });
  };

  return (
    <React.Fragment>
      <div className="loginBackground">
        <div className="loginContainer">
          <Row gutter={12}>
            <Col span={12}>
              <div className="container-login-form">
                <div className="login-header-contaner">
                  <center>
                    <img src={icon} alt="icon" className="login-icon" />
                  </center>
                  <h1 className="position_login">CONTENT SERVER</h1>
                </div>
                <center>
                  <img src={image} alt="icon" className="login-img" />
                </center>
              </div>
            </Col>
            <Col span={12}>
              <Form
                className="login-form"
                layout="vertical"
                size="large"
                onFinish={onSubmit}
              >
                {/* =================== Email ================= */}
                <center>
                  <h1>LOGIN</h1>
                </center>
                <Row>
                  <Col xl={24} lg={24} sm={24} xs={24}>
                    <Form.Item
                      label="Username"
                      name="username"
                      rules={[
                        {
                          required: true,
                          message: "Your username is invalided!",
                        },
                      ]}
                    >
                      <Input className="academyInputLarge" />
                    </Form.Item>
                  </Col>

                  {/* =================== Password ================= */}
                  <Col xl={24} lg={24} sm={24} xs={24}>
                    <Form.Item
                      label="Password"
                      name="password"
                      rules={[
                        {
                          required: true,
                          message: "Please input your password!",
                        },
                      ]}
                    >
                      <Input.Password
                        type="password"
                        className="academyInputLarge"
                      />
                    </Form.Item>
                  </Col>
                </Row>
                {/* =================== Button Submit ================= */}
                <Form.Item>
                  <Button
                    type="primary"
                    htmlType="submit"
                    className="login-form-button"
                  >
                    LOGIN
                  </Button>
                </Form.Item>
                <center>
                  <div className="footer-icon-container">
                    <p>powered by KOOMPI</p>
                    {/* <img
                      src={footerIcon}
                      alt="koompi logo"
                      className="footer-login-icon"
                    /> */}
                  </div>
                </center>
              </Form>
            </Col>
          </Row>
        </div>
      </div>
    </React.Fragment>
  );
};

export default Login;
