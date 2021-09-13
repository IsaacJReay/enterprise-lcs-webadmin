import React from "react";
import { Row, Col, Form, Input, Button, Checkbox } from "antd";

const Backup = () => {
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
          <h1>Backup Config</h1>
        </div>
        <hr />
        <div className="backup-container">
          <Form {...layout}>
            <Form.Item
              label="Name"
              name="name"
              rules={[
                {
                  required: true,
                  message: "Name is required!",
                },
              ]}
            >
              <Input size="large" className="label-info" />
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

            <Form.Item>
              <Button
                type="primary"
                htmlType="submit"
                className="button-apply"
                size="large"
              >
                Export
              </Button>
            </Form.Item>
          </Form>
        </div>
      </div>
    </React.Fragment>
  );
};

export default Backup;
