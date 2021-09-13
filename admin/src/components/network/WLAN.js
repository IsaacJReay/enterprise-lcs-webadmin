import React, { useState, useEffect } from "react";
import {
  Layout,
  Col,
  Row,
  Select,
  Button,
  Input,
  Form,
  message,
  Spin,
} from "antd";
import axios from "axios";

const { Content } = Layout;
const { Option } = Select;

const moment = require("moment-timezone");
const getToken = localStorage.getItem("token");
const auth = {
  Authorization: "Bearer " + getToken,
};

const WLANSetting = () => {
  const [form] = Form.useForm();
  // ---state -------
  const moments = moment.tz.names();
  const defalutTime = moment.tz.guess();
  const [pickValue, setPickValue] = useState(defalutTime);
  const [loading, setLoading] = useState(false);

  // ------layot form ---------
  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  const onChange = (value) => {
    setPickValue(value);
  };

  // ----------get data -------------

  useEffect(async () => {
    setLoading(true);
    await axios({
      method: "GET",
      url: "http://10.42.0.188:8080/private/api/settings/wirelessnetwork/status",
      headers: {
        "content-type": "application/json",
        ...auth,
      },
    })
      .then((res) => {
        const {
          router_ip,
          range_start,
          netmask,
          range_end,
          dns,
          default_lease,
          max_lease,
        } = res.data;
        form.setFieldsValue({
          router: router_ip,
          netmask: netmask,
          start_range: range_start,
          end_range: range_end,
          dns: dns,
          default_time: default_lease,
          max_time: max_lease,
        });
        setLoading(false);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  // ------- apply button ---------
  const handleApply = (data) => {
    const inputData = {
      router_ip: data.router,
      netmask: data.netmask,
      range_start: data.start_range,
      range_end: data.end_range,
      dns: data.dns,
      default_lease: data.default_time,
      max_lease: data.max_time,
      timezone: pickValue,
    };

    axios
      .post(
        "http://10.42.0.188:8080/private/api/settings/wirelessnetwork",
        inputData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error(err.response.data.reason);
      });
  };

  if (loading) {
    return (
      <div className="spin">
        <Spin />
      </div>
    );
  }

  return (
    <React.Fragment>
      <Content>
        <Row gutter={[32, 32]}>
          <Col span={16}>
            <Form {...layout} onFinish={handleApply} form={form}>
              <div className="container">
                <div className="container-header">
                  <h1>WLAN Setting</h1>
                </div>
                <hr />

                <div className="desc-container-banner">
                  <Form.Item
                    label="Router Address"
                    name="router"
                    rules={[
                      {
                        required: true,
                        message: "Input Router Address!",
                      },
                    ]}
                  >
                    <Input
                      size="large"
                      placeholder="0.0.0.0"
                      className="label-info"
                    />
                  </Form.Item>
                  <Form.Item
                    label="Netmask"
                    name="netmask"
                    rules={[
                      {
                        required: true,
                        message: "Input Netmask!",
                      },
                    ]}
                  >
                    <Input
                      size="large"
                      placeholder="0.0.0.0"
                      className="label-info"
                    />
                  </Form.Item>
                  <Form.Item label="Range " style={{ marginBottom: 0 }}>
                    <Form.Item
                      name="start_range"
                      rules={[
                        { required: true, message: "Input Start Range!" },
                      ]}
                      style={{
                        display: "inline-block",
                        width: "calc(50% - 8px)",
                      }}
                    >
                      <Input placeholder="0.0.0.0" size="large" />
                    </Form.Item>
                    <Form.Item
                      name="end_range"
                      rules={[{ required: true, message: "Input End Range!" }]}
                      style={{
                        display: "inline-block",
                        width: "calc(50% - 8px)",
                        margin: "0 8px",
                      }}
                    >
                      <Input placeholder="0.0.0.0" size="large" />
                    </Form.Item>
                  </Form.Item>
                  <Form.Item
                    label="DNS "
                    name="dns"
                    rules={[
                      {
                        required: true,
                        message: "Input DNS!",
                      },
                    ]}
                  >
                    <Input
                      size="large"
                      placeholder="0.0.0.0"
                      className="label-info"
                    />
                  </Form.Item>
                  <Form.Item
                    label="Default Lease time"
                    name="default_time"
                    rules={[
                      {
                        required: true,
                        message: "Input Default Lease time!",
                      },
                    ]}
                  >
                    <Input
                      size="large"
                      placeholder="0000"
                      className="label-info"
                    />
                  </Form.Item>
                  <Form.Item
                    label="Max Lease time"
                    name="max_time"
                    rules={[
                      {
                        required: true,
                        message: "Input Max Lease time!",
                      },
                    ]}
                  >
                    <Input
                      size="large"
                      placeholder="0000"
                      className="label-info"
                    />
                  </Form.Item>
                  <Form.Item label="Timezone ">
                    <Select
                      defaultValue={defalutTime}
                      size="large"
                      className="select-Option"
                      showSearch
                      optionFilterProp="children"
                      onChange={onChange}
                    >
                      {moments.map((res) => {
                        return <Option value={res}>{res}</Option>;
                      })}
                    </Select>
                  </Form.Item>
                </div>
                <Form.Item>
                  <Button
                    type="primary"
                    htmlType="submit"
                    className="button-apply"
                    size="large"
                  >
                    Apply
                  </Button>
                </Form.Item>
              </div>
            </Form>
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

export default WLANSetting;
