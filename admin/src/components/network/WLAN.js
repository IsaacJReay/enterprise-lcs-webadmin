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
  Space,
} from "antd";
import axios from "axios";
import { IoIosHelpCircle } from "react-icons/io";

const { Content } = Layout;
const { Option } = Select;

const moment = require("moment-timezone");

const WLANSetting = () => {
  // ------token ------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

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

  useEffect(() => {
    setLoading(true);
    axios({
      method: "GET",
      url: `${baseUrl}/settings/wirelessnetwork/status`,
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
      .post(`${baseUrl}/settings/wirelessnetwork`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })

      .then((res) => {
        if ((res.statusCode = 200)) {
          setTimeout(() => {
            message.success("Successful!");
          }, 1000);
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
        <Row gutter={12}>
          <Col span={16}>
            <Form {...layout} onFinish={handleApply} form={form}>
              <div className="card">
                <div className="container">
                  <div className="container-header">
                    <h1>WLAN SETTING</h1>
                  </div>
                  <div className="desc-container-banner2">
                    <Row gutter={[0, 24]}>
                      <Col span={24}>
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
                      </Col>
                      <Col span={24}>
                        {" "}
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
                      </Col>
                      <Col span={24}>
                        {" "}
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
                            rules={[
                              { required: true, message: "Input End Range!" },
                            ]}
                            style={{
                              display: "inline-block",
                              width: "calc(50% - 8px)",
                              margin: "0 8px",
                            }}
                          >
                            <Input placeholder="0.0.0.0" size="large" />
                          </Form.Item>
                        </Form.Item>
                      </Col>
                      <Col span={24}>
                        {" "}
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
                      </Col>
                      <Col span={24}>
                        {" "}
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
                      </Col>
                      <Col span={24}>
                        {" "}
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
                      </Col>
                      <Col span={24}>
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
                      </Col>
                    </Row>
                  </div>
                  <Form.Item>
                    <Button
                      type="primary"
                      htmlType="submit"
                      className="button-apply3"
                      size="large"
                    >
                      SAVE & APPLY
                    </Button>
                  </Form.Item>
                </div>
              </div>
            </Form>
          </Col>
          <Col span={8}>
            <div className="card2">
              <div className="container">
                <div className="container-header">
                  <Space>
                    <h1>HELPS</h1>
                    <IoIosHelpCircle className="icon-help" />
                  </Space>
                </div>
                <div>
                  <p>
                    You can configure the IP parameters of LAN on this page.
                  </p>
                  <ul>
                    <li>
                      <strong>MAC Address</strong> - The physical address of the
                      LAN ports, as seen from the LAN. The value cannot be
                      changed.
                    </li>
                    <li>
                      <strong>IP Address</strong> - Enter the IP address of your
                      Router in dotted-decimal notation (factory default -
                      192.168.0.1).
                    </li>
                    <li>
                      <strong>Subnet Musk</strong> - An address code that
                      determines the size of the network. Usually it is
                      255.255.255.0. Note:
                    </li>
                  </ul>
                  <p>
                    If you change the LAN IP address, you must use the new IP
                    address to login to the Router.
                  </p>
                  <p>
                    If the new LAN IP address you set is not in the same subnet
                    with the previous one, the IP Address pool in the DHCP
                    server will be configured automatically, but the Virtual
                    Server and DMZ Host will not take effect until they are
                    re-configured
                  </p>
                  <p>Click the Apply button to save your settings.</p>
                </div>
              </div>
            </div>
          </Col>
        </Row>
      </Content>
    </React.Fragment>
  );
};

export default WLANSetting;
