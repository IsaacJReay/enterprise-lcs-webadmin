import React from "react";
import { Row, Col, Layout, Form, Avatar, Input, Button } from "antd";
import Avatar1 from "../../assets/images/avatar/avatar.png";

const { Content } = Layout;

const UserAccount = () => {
  // ------layot form ---------
  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  return (
    <React.Fragment>
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <div className="container">
              <div className="container-header">
                <h1>Change Password</h1>
              </div>
              <hr />
              <Form {...layout}>
                <div className="user-account-contanier">
                  <Avatar size={100} className="navbar-avatar" src={Avatar1} />
                  <h2>Hello world</h2>
                </div>
                <Form.Item
                  label="Current Password"
                  name="current_password"
                  rules={[
                    {
                      required: true,
                      message: "Input Current Password!",
                    },
                  ]}
                >
                  <Input.Password className="label-info" size="large" />
                </Form.Item>
                <Form.Item
                  label="New Password"
                  name="new_password"
                  rules={[
                    {
                      required: true,
                      message: "Input New Password!",
                    },
                  ]}
                >
                  <Input.Password className="label-info" size="large" />
                </Form.Item>
                <Form.Item
                  label="Confirm Password"
                  name="confirm_password"
                  rules={[
                    {
                      required: true,
                      message: "Confirm Password is required!",
                    },
                    ({ getFieldValue }) => ({
                      validator(_, value) {
                        if (!value || getFieldValue("new_password") === value) {
                          return Promise.resolve();
                        }
                        return Promise.reject(
                          new Error(
                            "The two passwords that you entered do not match!"
                          )
                        );
                      },
                    }),
                  ]}
                >
                  <Input.Password className="label-info" size="large" />
                </Form.Item>
                <Form.Item>
                  <Button
                    type="primary"
                    htmlType="submit"
                    className="button-apply"
                    size="large"
                  >
                    Apply
                  </Button>
                </Form.Item>
              </Form>
            </div>
          </Col>
          <Col span={8}>
            <div className="container">
              <div className="container-header">
                <h1>Desciptions</h1>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default UserAccount;
