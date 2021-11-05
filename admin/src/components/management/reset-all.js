import React, { useState } from "react";
import { Form, Checkbox, Button, Row, Col, Layout, message } from "antd";
import axios from "axios";

const { Content } = Layout;

const ResetAll = () => {
  const [loading, setLoading] = useState(false);
  const [checked, setChecked] = useState(false);
  // ------token ------

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
      url: "http://10.42.0.188:8080/private/api/settings/reset",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        if (res.data.operation_status === "Failed") {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        } else {
          setLoading(true);
          message.success("Successful!");
          window.location.replace("/logout");
          setLoading(false);
        }
      })
      .catch((err) => console.log(err));
  };

  return (
    <React.Fragment>
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <div className="container">
              <div className="container-header">
                <h1>Factory Defaults</h1>
              </div>
              <hr />
              <div className="reset-container">
                <h3>
                  Click the following button to reset all configuration settings
                  to their default values.
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
                      className="button-apply2"
                      disabled={checked !== true}
                    >
                      Reset
                    </Button>
                  </Form.Item>
                </Form>
              </div>
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

export default ResetAll;
