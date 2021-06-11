import React from "react";
import { Input, Button, Form, Col, Row } from "antd";

const Login = () => {
  return (
    <React.Fragment>
      <div className="loginBackground">
        <div className="loginContainer">
          <div className="background_image">
            <h1 className="position_login">CONTENT SERVER | LOGIN</h1>
          </div>
          <Form className="login-form" layout="vertical" size="large">
            {/* =================== Email ================= */}
            <Col xl={24} lg={24} sm={24} xs={24}>
              <Form.Item
                label="Email"
                name="email"
                rules={[
                  {
                    type: "email",
                    message: "Your email is invalided!",
                  },
                  {
                    required: true,
                    message: "Please input your email!",
                  },
                ]}
              >
                <Input
                  autoFocus={true}
                  type="email"
                  className="academyInputLarge"
                  //   value={email || ""}
                  //   onChange={({ target: { value } }) => setEmail(value)}
                />
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
                <Input.Password type="password" className="academyInputLarge" />
              </Form.Item>
            </Col>
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
