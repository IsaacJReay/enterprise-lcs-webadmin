import React, { useState } from "react";
import { Form, Checkbox, Button, Row, Col, Layout, message, Space } from "antd";
import axios from "axios";
import { IoIosHelpCircle } from "react-icons/io";

const { Content } = Layout;

const ResetAll = () => {
  const [, setLoading] = useState(false);
  const [checked, setChecked] = useState(false);
  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // -------onchange ------------

  const onChange = (e) => {
    setChecked(e.target.checked);
  };

  // -----------on Apply ----------

  const handleApply = () => {
    axios({
      method: "POST",
      url: `${baseUrl}/settings/reset`,
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        if ((res.statusCode = 200)) {
          setLoading(true);
          message.success("Successful!");
          window.location.replace("/logout");
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  return (
    <React.Fragment>
      <Content>
        <Row gutter={12}>
          <Col span={16}>
            <div className="card">
              <div className="container">
                <div className="container-header">
                  <h1>FACTORY DEFAULT</h1>
                </div>
                <div className="reset-container">
                  <h3>
                    Click the following button to reset all configuration
                    settings to their default values.
                  </h3>
                  <Form onFinish={handleApply}>
                    <Form.Item name="check">
                      <Checkbox style={{ color: "red " }} onChange={onChange}>
                        Confirm that all changed settings will be lost when
                        defaults are restored.
                      </Checkbox>
                    </Form.Item>
                    <Form.Item>
                      <Button
                        size="large"
                        type="primary"
                        htmlType="submit"
                        className="button-reset"
                        disabled={checked !== true}
                      >
                        RESET
                      </Button>
                    </Form.Item>
                  </Form>
                </div>
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
                  <h1>Factory Defaults Help</h1>

                  <p>
                    Click the Restore button to reset all configuration settings
                    to their default values.
                  </p>
                  <ul>
                    <li>
                      <strong>Default User Name</strong> - admin
                    </li>
                    <li>
                      <strong>Default Password </strong> - admin
                    </li>
                    <li>
                      <strong>Default IP Address</strong> - 192.168.0.1
                    </li>
                    <li>
                      <strong>Default Subnet Mask</strong> - 255.255.255.0
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default ResetAll;
