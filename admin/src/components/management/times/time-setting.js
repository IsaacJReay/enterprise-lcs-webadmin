import React, { useState, useEffect } from "react";
import { Row, Col, Layout, Radio, Form } from "antd";
import Automaticaly from "./auto";
import CustomeTime from "./manaul";
import axios from "axios";

const { Content } = Layout;

const TimeSetting = () => {
  const [pick, setPick] = useState();
  const [loading, setLoading] = useState(false);
  const [items, setItems] = useState({});
  const [form] = Form.useForm();

  // ------token ------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  const handleChange = (e) => {
    return setPick(e.target.value);
  };

  // ----------get data -------------

  useEffect(async () => {
    await axios({
      method: "GET",
      url: "http://10.42.0.188:8080/private/api/settings/time/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        setLoading(true);
        const { ntp_status } = res.data;
        form.setFieldsValue({ ntp_status: ntp_status });
        setPick(res.data.ntp_status);
        setItems(res.data);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

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
                <Form form={form}>
                  <Form.Item name="ntp_status">
                    <Radio.Group
                      onChange={handleChange}
                      valuePropName="checked"
                    >
                      <Radio value={true}>
                        Automatically synchronize with an Internet time server
                      </Radio>
                      <Radio className="custom-radio" value={false}>
                        Custom
                      </Radio>
                    </Radio.Group>
                  </Form.Item>
                </Form>
                <div className="time-container">
                  <Automaticaly pick={pick} items={items} />
                </div>
                <div className="time-container2">
                  <CustomeTime pick={pick} items={items} />
                </div>
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
