import React, { useState } from "react";
import { Row, Col, Layout, Radio, Form } from "antd";
import Automaticaly from "./auto";
import CustomeTime from "./manaul";

const { Content } = Layout;

const TimeSetting = () => {
  const [pick, setPick] = useState("auto");

  const handleChange = (e) => {
    return setPick(e.target.value);
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
                <Form>
                  <Radio.Group onChange={handleChange} defaultValue="auto">
                    <Form.Item>
                      <Radio value="auto">
                        Automatically synchronize with an Internet time server
                      </Radio>
                    </Form.Item>
                    <Form.Item>
                      <Automaticaly pick={pick} />
                    </Form.Item>
                    <Form.Item>
                      <Radio value="manual">Custom</Radio>
                    </Form.Item>
                    <Form.Item>
                      <CustomeTime pick={pick} />
                    </Form.Item>
                  </Radio.Group>
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

export default TimeSetting;
