import React, { useState } from "react";
import { Input, Button, Form, Col, Row, message } from "antd";
import axios from "axios";

const Login = () => {
  const [loading, setLoading] = useState(false);

  const onSubmit = (data) => {
    const adminLogin = {
      username: data.username,
      password: data.password,
    };

    axios
      .post(
        "http://10.42.0.188:8080/private/api/user/login",
        adminLogin,
        setLoading(true)
      )

      .then(async (res) => {
        await localStorage.setItem("token", res.data.token);
        if (res.data.operation_status === "Success") {
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
          <div className="background_image">
            <h1 className="position_login">CONTENT SERVER | LOGIN</h1>
          </div>
          <Form
            className="login-form"
            layout="vertical"
            size="large"
            onFinish={onSubmit}
          >
            {/* =================== Email ================= */}
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
          </Form>
        </div>
      </div>
    </React.Fragment>
  );
};

export default Login;
