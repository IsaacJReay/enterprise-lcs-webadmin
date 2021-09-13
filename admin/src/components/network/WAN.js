import React, { useState } from "react";
import { Layout, Col, Row, Select, Form } from "antd";
import WANStatic from "./wan-static";
import WANDynamic from "./wan-dynamic";

const { Content } = Layout;
const { Option } = Select;

const WANSetting = () => {
  const [values, setValues] = useState("Dynamic");

  const handleChange = (value) => {
    return setValues(value);
  };
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
                <h1>WAN Setting</h1>
              </div>
              <hr />
              <div className="desc-container-banner">
                <Form {...layout}>
                  <Form.Item label="WAN Connection type">
                    <Row gutter={[32, 32]}>
                      <Col>
                        <Select
                          onChange={handleChange}
                          defaultValue="Dynamic"
                          size="large"
                          className="select-option"
                        >
                          <Option value="Dynamic">Dynamic</Option>
                          <Option value="Static">Static</Option>
                        </Select>
                      </Col>
                    </Row>
                  </Form.Item>
                </Form>
              </div>

              {/* ----------form------ */}
              <div className="desc-container-banner">
                {values === "Dynamic" ? <WANDynamic /> : <WANStatic />}
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

export default WANSetting;
