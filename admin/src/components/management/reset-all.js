import React from "react";
import { Form, Checkbox, Button, Row, Col, Layout } from "antd";

const { Content } = Layout;

const ResetAll = () => {
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
                <Form>
                  <Form.Item name="check">
                    <Checkbox style={{ color: "red " }}>
                      Confirm that all changed settings will be lost when
                      defaults are restored.
                    </Checkbox>
                  </Form.Item>
                  <Form.Item>
                    <Button
                      size="large"
                      type="primary"
                      typeof="submit"
                      className="button-apply2"
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
