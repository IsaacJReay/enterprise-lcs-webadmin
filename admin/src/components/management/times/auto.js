import React, { useState } from "react";
import { Form, Select, Button, Row, Col } from "antd";

const { Option } = Select;
const moment = require("moment-timezone");

const Automaticaly = ({ pick }) => {
  const moments = moment.tz.names();
  const defalutTime = moment.tz.guess();
  //   const [pickValue, setPickValue] = useState(defalutTime);

  //   const onChange = (value) => {
  //     setPickValue(value);
  //   };

  return (
    <React.Fragment>
      <Form>
        <Form.Item>
          {pick === "auto" ? (
            <div>
              <Row gutter={[12, 12]}>
                <Col span={20}>
                  <Form.Item label="Zone">
                    <Select
                      defaultValue={defalutTime}
                      size="large"
                      className="select-Option"
                      showSearch
                      optionFilterProp="children"
                    >
                      {moments.map((res) => {
                        return <Option value={res}>{res}</Option>;
                      })}
                    </Select>
                  </Form.Item>
                </Col>
                <Col span={4}>
                  <Form.Item>
                    <Button
                      size="large"
                      type="primary"
                      typeof="button"
                      className="button-update"
                    >
                      Update
                    </Button>
                  </Form.Item>
                </Col>
              </Row>
            </div>
          ) : (
            <Row gutter={[12, 12]}>
              <Col span={20}>
                <Form.Item label="Zone">
                  <Select
                    defaultValue={defalutTime}
                    size="large"
                    className="select-Option"
                    showSearch
                    optionFilterProp="children"
                    disabled
                  >
                    {moments.map((res) => {
                      return <Option value={res}>{res}</Option>;
                    })}
                  </Select>
                </Form.Item>
              </Col>
              <Col span={4}>
                <Form.Item>
                  <Button
                    size="large"
                    type="primary"
                    typeof="button"
                    className="button-update"
                    disabled
                  >
                    Update
                  </Button>
                </Form.Item>
              </Col>
            </Row>
          )}
        </Form.Item>
        <Form.Item label="Time">
          <time className="date-time">10:01:01 AM</time>
        </Form.Item>
        <Form.Item label="Date">
          <time className="date-time">2021-05-21</time>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default Automaticaly;
