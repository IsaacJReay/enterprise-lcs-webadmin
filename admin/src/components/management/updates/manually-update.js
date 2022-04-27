import React from "react";
import { Form, Button, Input, Space } from "antd";

const ManaullyUpdate = () => {
  return (
    <React.Fragment>
      <div className="container-manually">
        <Form>
          <Form.Item>
            <Space>
              <Form.Item
                label="Path"
                name="name"
                rules={[
                  {
                    required: true,
                    message: "Path is required!",
                  },
                ]}
              >
                <Input size="large" className="input-info" />
              </Form.Item>
              <Form.Item>
                <Button size="large" className="browse-buttons">
                  Browse
                </Button>
              </Form.Item>
            </Space>
          </Form.Item>
          <Form.Item>
            <Button
              htmlType="submit"
              size="large"
              type="primary"
              className="button-apply5"
            >
              Update
            </Button>
          </Form.Item>
        </Form>
      </div>
    </React.Fragment>
  );
};

export default ManaullyUpdate;
