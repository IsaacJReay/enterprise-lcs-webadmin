import React, { useState, useEffect } from "react";
import { Form, Input, Button, message, Spin, Row, Col } from "antd";
import axios from "axios";

const getToken = localStorage.getItem("token");
const baseUrl = process.env.REACT_APP_API_URL;

const WANStatic = ({ wan, fetchData }) => {
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();

  const auth = {
    Authorization: "Bearer " + getToken,
  };

  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  const { internet_ip, gateway, netmask, dns } = wan;

  useEffect(
    async () =>
      await form.setFieldsValue({
        internet_ip: internet_ip,
        gateway: gateway,
        netmask: netmask,
        dns: dns,
      })
  );

  const handleApply = async (data) => {
    const inputData = {
      internet_ip: data.internet_ip,
      netmask: data.netmask,
      gateway: data.gateway,
      dns: data.dns,
    };

    await axios
      .post(` ${baseUrl}/settings/wirednetwork/static`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if ((res.statusCode = 200)) {
          message.success("Successful!");
          fetchData();
        } else {
          message.error("Operation Failed! ");
        }
      })
      .catch((err) => {
        console.log(err);
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
      <Form {...layout} onFinish={handleApply} form={form}>
        <Row gutter={[0, 24]}>
          <Col span={24}>
            {" "}
            <Form.Item
              label="Internet IP"
              name="internet_ip"
              rules={[
                {
                  required: true,
                  message: "Input Internet IP!",
                },
              ]}
            >
              <Input size="large" className="label-info" />
            </Form.Item>
          </Col>
          <Col span={24}>
            {" "}
            <Form.Item
              label="Subnet Mask"
              name="netmask"
              rules={[
                {
                  required: true,
                  message: "Input Subnet Mask!",
                },
              ]}
            >
              <Input size="large" className="label-info" />
            </Form.Item>
          </Col>

          <Col span={24}>
            {" "}
            <Form.Item
              label="Default Getway"
              name="gateway"
              rules={[
                {
                  required: true,
                  message: "Input Default Getway!",
                },
              ]}
            >
              <Input size="large" className="label-info" />
            </Form.Item>
          </Col>
          <Col span={24}>
            <Form.Item
              label="DNS"
              name="dns"
              rules={[
                {
                  required: true,
                  message: "Input DNS!",
                },
              ]}
            >
              <Input size="large" className="label-info" />
            </Form.Item>
          </Col>
        </Row>
        <Form.Item>
          <Button
            type="primary"
            htmlType="submit"
            className="button-apply"
            size="large"
          >
            SAVE & APPLY
          </Button>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default WANStatic;
