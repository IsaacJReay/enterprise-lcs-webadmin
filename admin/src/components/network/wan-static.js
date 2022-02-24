import React, { useState, useEffect } from "react";
import { Form, Input, Button, message, Spin } from "antd";
import axios from "axios";

const getToken = localStorage.getItem("token");
const baseUrl = process.env.REACT_APP_API_URL;

const WANStatic = () => {
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

  useEffect(() => {
    async function fetchData() {
      await axios({
        method: "GET",
        url: `${baseUrl}/settings/wirednetwork/status`,
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
        .then((res) => {
          const { internet_ip, gateway, netmask, dns } =
            res.data.wired_network_param;
          form.setFieldsValue({
            internet_ip: internet_ip,
            gateway: gateway,
            netmask: netmask,
            dns: dns,
          });
          setLoading(false);
          setTimeout(() => {
            setLoading(false);
          }, 1000);
        })
        .catch((err) => console.log(err));
    }
    fetchData();
  }, []);

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
        if (res.data.operation_status === "Success") {
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
      </Form>
    </React.Fragment>
  );
};

export default WANStatic;
