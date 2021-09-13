import React from "react";
import { Row, Col, Form, Input, Button, Checkbox } from "antd";

const Restore = () => {
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
      <div className="container">
        <div className="container-header">
          <h1>Restore Config</h1>
        </div>
        <hr />
        <Form {...layout}>
          <Form.Item className="restore-settting">
            <Row gutter={[12, 12]}>
              <Col>
                <Form.Item
                  label="File"
                  name="file"
                  rules={[
                    {
                      required: true,
                      message: "File is required!",
                    },
                  ]}
                >
                  <Input size="large" style={{ width: 315 }} />
                </Form.Item>
              </Col>
              <Col>
                <Form.Item>
                  <Button
                    className="browse-buttons"
                    size="large"
                    htmlType="button"
                  >
                    Browse
                  </Button>
                </Form.Item>
              </Col>
            </Row>
          </Form.Item>
          <Form.Item
            label="Password"
            name="password"
            rules={[
              {
                required: true,
                message: "Input Password!",
              },
            ]}
          >
            <Input.Password className="label-info" size="large" />
          </Form.Item>
          <Form.Item name="confirm">
            <Checkbox className="restore-checkbox">Confirm</Checkbox>
          </Form.Item>

          <Form.Item>
            <Button
              type="primary"
              htmlType="submit"
              className="button-apply"
              size="large"
            >
              Import
            </Button>
          </Form.Item>
        </Form>
      </div>
    </React.Fragment>
  );
};

export default Restore;
