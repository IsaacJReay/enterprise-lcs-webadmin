import React, { useState, useEffect } from "react";
import {
  Row,
  Col,
  Layout,
  Form,
  Avatar,
  Input,
  Button,
  message,
  Space,
} from "antd";
import Avatar1 from "../../assets/images/avatar/avatar.png";
import axios from "axios";
import { IoIosHelpCircle } from "react-icons/io";

const { Content } = Layout;

const UserAccount = () => {
  const [, setLoading] = useState(false);
  const [form] = Form.useForm();
  const [user, setUser] = useState({});

  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // ------layot form ---------
  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  // -----------on Apply ----------

  const handleApply = (data) => {
    const inputData = {
      old_password: data.current_password,
      new_password: data.new_password,
    };

    axios
      .post(`${baseUrl}/user/password`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
          setLoading(true);
          window.location.replace("/logout");
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  // ------query user -----------
  useEffect(() => {
    setLoading(true);
    axios({
      method: "GET",
      url: `${baseUrl}/user/query`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setUser(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>ACCOUNT SETTING</h1>
                </div>
                <div className="user-information">
                  <span>
                    After the password is up to date the site will logout!
                  </span>
                </div>
                <Form {...layout} form={form} onFinish={handleApply}>
                  <div className="user-account-contanier">
                    <Avatar
                      size={100}
                      className="navbar-avatar"
                      src={Avatar1}
                    />
                    <h2>{user.username}</h2>
                  </div>

                  <div className="label-container">
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
                            if (
                              !value ||
                              getFieldValue("new_password") === value
                            ) {
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
                  </div>
                  <Form.Item>
                    <Button
                      type="primary"
                      htmlType="submit"
                      className="button-apply4"
                      size="large"
                    >
                      SAVE & APPLY
                    </Button>
                  </Form.Item>
                </Form>
              </div>
            </div>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
                </div>
                <div>
                  <h1>Account Setting</h1>
                  <p>
                    It is strongly recommended that you change the factory
                    default user name and password of this device. All users who
                    try to access this device's web-based utility will be
                    prompted for this device's user name and password.
                  </p>
                  <p>
                    <strong>Note:</strong> The new user name and password must
                    not exceed 15 characters in length and must not include any
                    spaces. Enter the new Password twice to confirm it.
                  </p>
                  <p>Click the apply button when finished.</p>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default UserAccount;
