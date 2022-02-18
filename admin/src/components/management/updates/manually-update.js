import React from "react";
import { Form, Button, Input, Row, Col } from "antd";

const ManaullyUpdate = () => {
  return (
    <React.Fragment>
         <div className="card">
      <div className="container">
        <div className="container-header">
          <h1>Manually Update</h1>
        </div>
        <hr />
        <div className="container-manually">
          <Form>
            <Form.Item>
              <Row gutter={[24, 24]}>
                <Col span={16}>
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
                    <Input size="large" />
                  </Form.Item>
                </Col>
                <Col span={8}>
                  <Form.Item>
                    <Button
                      size="large"
                      htmlType="submit"
                      className="browse-buttons"
                    >
                      Browse
                    </Button>
                  </Form.Item>
                </Col>
              </Row>
            </Form.Item>
            <Form.Item>
              <Button
                htmlType="submit"
                size="large"
                type="primary"
                className="button-update"
              >
                Update
              </Button>
            </Form.Item>
          </Form>
        </div>
        </div>
      </div>
    </React.Fragment>
  );
};

export default ManaullyUpdate;
